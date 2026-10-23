#!/usr/bin/env python3
"""Acquire primary world-state papers and pinned source archives, without builds.

Run `python3 papers/survey/world-state/acquire.py papers`, `refs`, or `restore`.
Re-running papers preserves any reading notes recorded in bibliography.tsv.
"""
from concurrent.futures import ThreadPoolExecutor, as_completed
import csv
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path, PurePosixPath
import subprocess
import sys
import tarfile
import urllib.request

ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).resolve().parent
ARCH = ROOT/'quarantine/archives/survey-world-state'
SRC = ROOT/'quarantine/sources/survey-world-state'
DATE = '2026-10-23'
PAPERS = [
('lamport78','causality','Time, Clocks, and the Ordering of Events in a Distributed System','Leslie Lamport',1978,'CACM 21(7)','10.1145/359545.359563','https://lamport.azurewebsites.net/pubs/time-clocks.pdf','papers/pdfs/lamport-time-clocks.pdf'),
('snapshot85','snapshots','Distributed Snapshots: Determining Global States of Distributed Systems','K. Mani Chandy; Leslie Lamport',1985,'TOCS 3(1)','10.1145/214451.214456','https://lamport.azurewebsites.net/pubs/chandy.pdf',''),
('fidge88','causality','Timestamps in Message-Passing Systems That Preserve the Partial Ordering','Colin J. Fidge',1988,'Australian Computer Science Communications 10(1)','','https://zoo.cs.yale.edu/classes/cs426/2012/lab/bib/fidge88timestamps.pdf',''),
('mattern89','causality','Virtual Time and Global States of Distributed Systems','Friedemann Mattern',1989,'Parallel and Distributed Algorithms','','https://pages.cs.wisc.edu/~remzi/Classes/739/Fall2018/Papers/mattern89.pdf',''),
('global93','snapshots','Consistent Global States of Distributed Systems: Fundamental Concepts and Mechanisms','Ozalp Babaoglu; Keith Marzullo',1993,'Technical Report UBLCS-93-1','','https://www.cs.cornell.edu/courses/cs5414/2017fa/papers/chapt4.pdf',''),
('hlc14','causality','Logical Physical Clocks and Consistent Snapshots in Globally Distributed Databases','Sandeep Kulkarni; Murat Demirbas; Deepak Madeppa; Bharadwaj Avva; Marcelo Leone',2014,'University at Buffalo Technical Report 2014-04','','https://cse.buffalo.edu/tech-reports/2014-04.pdf',''),
('spanner12','time','Spanner: Google\'s Globally-Distributed Database','James C. Corbett; Jeffrey Dean; Michael Epstein; Andrew Fikes; Christopher Frost; JJ Furman; Sanjay Ghemawat; Andrey Gubarev; Christopher Heiser; Peter Hochschild; Wilson Hsieh; Sebastian Kanthak; Eugene Kogan; Hongyi Li; Alexander Lloyd; Sergey Melnik; David Mwaura; David Nagle; Sean Quinlan; Rajesh Rao; Lindsay Rolig; Yasushi Saito; Michal Szymaniak; Christopher Taylor; Ruth Wang; Dale Woodford',2012,'OSDI 2012','','https://www.usenix.org/system/files/conference/osdi12/osdi12-final-16.pdf',''),
('taxonomy85','temporal','A Taxonomy of Time in Databases','Richard Snodgrass; Ilsoo Ahn',1985,'SIGMOD 1985','10.1145/318898.318921','https://www2.cs.arizona.edu/~rts/pubs/SIGMOD85.pdf',''),
('unification93','temporal','Unification of Temporal Data Models','Christian S. Jensen; Michael D. Soo; Richard T. Snodgrass',1993,'ICDE 1993','10.1109/ICDE.1993.344056','https://www2.cs.arizona.edu/~rts/pubs/ICDE93.pdf',''),
('eventsourcing05','events','Event Sourcing','Martin Fowler',2005,'Original essay','','https://martinfowler.com/eaaDev/EventSourcing.html','reference-only'),
('immutability15','events','Immutability Changes Everything','Pat Helland',2015,'CIDR 2015','','https://www.cidrdb.org/cidr2015/Papers/CIDR15_Paper16.pdf',''),
('whywhere01','provenance','Why and Where: A Characterization of Data Provenance','Peter Buneman; Sanjeev Khanna; Wang-Chiew Tan',2001,'ICDT 2001','10.1007/3-540-44503-X_20','https://www.cis.upenn.edu/~sanjeev/papers/icdt01_data_provenance.pdf',''),
('semirings07','provenance','Provenance Semirings','Todd J. Green; Grigoris Karvounarakis; Val Tannen',2007,'PODS 2007','10.1145/1265530.1265535','https://www.cs.ucdavis.edu/~green/papers/pods07.pdf',''),
('provsurvey09','provenance','Provenance in Databases: Why, How, and Where','James Cheney; Laura Chiticariu; Wang-Chiew Tan',2009,'Foundations and Trends in Databases 1(4)','10.1561/1900000006','https://homepages.inf.ed.ac.uk/jcheney/publications/provdbsurvey.pdf',''),
('prov-dm13','provenance','PROV-DM: The PROV Data Model','Luc Moreau; Paolo Missier (editors); W3C Provenance Working Group',2013,'W3C Recommendation 30 April 2013','','https://www.w3.org/TR/2013/REC-prov-dm-20130430/','reference-only'),
('pass06','provenance','Provenance-Aware Storage Systems','Kiran-Kumar Muniswamy-Reddy; David A. Holland; Uri Braun; Margo Seltzer',2006,'USENIX ATC 2006','','https://www.usenix.org/legacy/events/usenix06/tech/full_papers/muniswamy-reddy/muniswamy-reddy.pdf',''),
('pass09','provenance','Layering in Provenance Systems','Kiran-Kumar Muniswamy-Reddy; Uri Braun; David A. Holland; Peter Macko; Diana Maclean; Daniel Margo; Margo Seltzer; Robin Smogor',2009,'USENIX ATC 2009','','https://www.usenix.org/legacy/events/usenix09/tech/full_papers/muniswamy-reddy/muniswamy-reddy.pdf',''),
('camflow17','provenance','Practical Whole-System Provenance Capture','Thomas Pasquier; Xueyuan Han; Mark Goldstein; Thomas Moyer; David Eyers; Margo Seltzer; Jean Bacon',2017,'SoCC 2017','10.1145/3127479.3129249','https://camflow.org/publications/socc-2017.pdf',''),
('camquery18','provenance','Runtime Analysis of Whole-System Provenance','Thomas Pasquier; Xueyuan Han; Thomas Moyer; Adam Bates; Olivier Hermant; David Eyers; Jean Bacon; Margo Seltzer',2018,'CCS 2018','10.1145/3243734.3243776','https://camflow.org/publications/ccs-2018.pdf',''),
('gprom14','provenance','A Generic Provenance Middleware for Database Queries, Updates, and Transactions','Bahareh Arab; Dieter Gawlick; Venkatesh Radhakrishnan; Hao Guo; Boris Glavic',2014,'USENIX TaPP 2014','','https://www.cs.uic.edu/~bglavic/dbgroup/assets/pdfpubls/AG14.pdf',''),
('crdt11','convergence','A Comprehensive Study of Convergent and Commutative Replicated Data Types','Marc Shapiro; Nuno Preguica; Carlos Baquero; Marek Zawirski',2011,'INRIA Research Report RR-7506','','https://inria.hal.science/inria-00555588/document',''),
('deltacrdt18','convergence','Delta State Replicated Data Types','Paulo Sergio Almeida; Ali Shoker; Carlos Baquero',2018,'JPDC 111 (2018); acquired arXiv:1603.01529v1 preprint (2016)','10.1016/j.jpdc.2017.08.003','https://arxiv.org/pdf/1603.01529',''),
('jsoncrdt17','convergence','A Conflict-Free Replicated JSON Datatype','Martin Kleppmann; Alastair R. Beresford',2017,'IEEE TPDS 28(10)','10.1109/TPDS.2017.2697382','https://www.cl.cam.ac.uk/~arb33/papers/KleppmannBeresford-CRDT-JSON-TPDS2017.pdf',''),
('localfirst19','convergence','Local-First Software: You Own Your Data, in spite of the Cloud','Martin Kleppmann; Adam Wiggins; Peter van Hardenberg; Mark McGranaghan',2019,'Onward! 2019','10.1145/3359591.3359737','https://martin.kleppmann.com/papers/local-first.pdf',''),
('calm20','coordination','Keeping CALM: When Distributed Consistency Is Easy','Joseph M. Hellerstein; Peter Alvaro',2020,'CACM 63(9) (2020); acquired arXiv:1901.01930v2 preprint (2019)','10.1145/3369736','https://arxiv.org/pdf/1901.01930',''),
('differential13','incremental','Differential Dataflow','Frank McSherry; Derek G. Murray; Rebecca Isaacs; Michael Isard',2013,'CIDR 2013','','https://www.cidrdb.org/cidr2013/Papers/CIDR13_Paper111.pdf',''),
('naiad13','incremental','Naiad: A Timely Dataflow System','Derek G. Murray; Frank McSherry; Rebecca Isaacs; Michael Isard; Paul Barham; Martin Abadi',2013,'SOSP 2013','10.1145/2517349.2522738','https://pages.cs.wisc.edu/~shivaram/cs744-readings/Naiad.pdf',''),
('dbsp23','incremental','DBSP: Automatic Incremental View Maintenance for Rich Query Languages','Mihai Budiu; Tej Chajed; Frank McSherry; Leonid Ryzhyk; Val Tannen',2023,'PVLDB 16(7)','10.14778/3587136.3587137','https://www.vldb.org/pvldb/vol16/p1601-budiu.pdf',''),
('rdd12','recovery','Resilient Distributed Datasets: A Fault-Tolerant Abstraction for In-Memory Cluster Computing','Matei Zaharia; Mosharaf Chowdhury; Tathagata Das; Ankur Dave; Justin Ma; Murphy McCauley; Michael J. Franklin; Scott Shenker; Ion Stoica',2012,'NSDI 2012','','https://www.usenix.org/system/files/conference/nsdi12/nsdi12-final138.pdf',''),
('dstreams13','recovery','Discretized Streams: Fault-Tolerant Streaming Computation at Scale','Matei Zaharia; Tathagata Das; Haoyuan Li; Timothy Hunter; Scott Shenker; Ion Stoica',2013,'SOSP 2013','10.1145/2517349.2522737','https://people.csail.mit.edu/matei/papers/2013/sosp_spark_streaming.pdf',''),
('millwheel13','streams','MillWheel: Fault-Tolerant Stream Processing at Internet Scale','Tyler Akidau; Alex Balikov; Kaya Bekiroglu; Slava Chernyak; Josh Haberman; Reuven Lax; Sam McVeety; Daniel Mills; Paul Nordstrom; Sam Whittle',2013,'PVLDB 6(11)','10.14778/2536222.2536229','https://www.vldb.org/pvldb/vol6/p1033-akidau.pdf',''),
('dataflow15','streams','The Dataflow Model: A Practical Approach to Balancing Correctness, Latency, and Cost in Massive-Scale, Unbounded, Out-of-Order Data Processing','Tyler Akidau; Robert Bradshaw; Craig Chambers; Slava Chernyak; Rafael J. Fernandez-Moctezuma; Reuven Lax; Sam McVeety; Daniel Mills; Frances Perry; Eric Schmidt; Sam Whittle',2015,'PVLDB 8(12)','10.14778/2824032.2824076','https://www.vldb.org/pvldb/vol8/p1792-Akidau.pdf',''),
('flink15','recovery','Lightweight Asynchronous Snapshots for Distributed Dataflows','Paris Carbone; Gyula Fora; Stephan Ewen; Seif Haridi; Kostas Tzoumas',2015,'arXiv:1506.08603','10.48550/arXiv.1506.08603','https://arxiv.org/pdf/1506.08603',''),
('watermarks21','streams','Watermarks in Stream Processing Systems: Semantics and Comparative Analysis of Apache Flink and Google Cloud Dataflow','Tyler Akidau; Edmon Begoli; Slava Chernyak; Fabian Hueske; Kathryn Knight; Kenneth Knowles; Daniel Mills; Dan Sotolongo',2021,'PVLDB 14(12)','10.14778/3476311.3476389','https://www.vldb.org/pvldb/vol14/p3135-begoli.pdf',''),
('rollback02','recovery','A Survey of Rollback-Recovery Protocols in Message-Passing Systems','E. N. Elnozahy; Lorenzo Alvisi; Yi-Min Wang; David B. Johnson',1999,'CMU-CS-99-148 technical report (journal version ACM Computing Surveys 34(3), 2002)','10.1145/568522.568525 (later journal version)','https://www.cs.rice.edu/~dbj/pubs/CMU-CS-99-148.pdf',''),
('verifiedprogress21','progress','Verified Progress Tracking for Timely Dataflow','Matthias Brun; Sara Decova; Andrea Lattuada; Dmitriy Traytel',2021,'ITP 2021','10.4230/LIPIcs.ITP.2021.10','https://drops.dagstuhl.de/storage/00lipics/lipics-vol193-itp2021/LIPIcs.ITP.2021.10/LIPIcs.ITP.2021.10.pdf',''),
('progress13','progress','Formal Analysis of a Distributed Algorithm for Tracking Progress','Martin Abadi; Frank McSherry; Derek G. Murray; Thomas L. Rodeheffer',2013,'FMOODS-FORTE 2013','10.1007/978-3-642-38592-6_2','https://www.microsoft.com/en-us/research/wp-content/uploads/2013/06/clock-verif2.pdf',''),
('clockproof13','progress','The Naiad Clock Protocol: Specification, Model Checking, and Correctness Proof','Tom Rodeheffer',2013,'MSR-TR-2013-20','','https://www.microsoft.com/en-us/research/wp-content/uploads/2013/02/paper.pdf',''),
('falkirk15','recovery','Falkirk Wheel: Rollback Recovery for Dataflow Systems','Michael Isard; Martin Abadi',2015,'arXiv:1503.08877v1 draft','10.48550/arXiv.1503.08877','https://arxiv.org/pdf/1503.08877',''),
('itc08','causality','Interval Tree Clocks: A Logical Clock for Dynamic Systems','Paulo Sergio Almeida; Carlos Baquero; Victor Fonte',2008,'OPODIS 2008','10.1007/978-3-540-92221-6_18','https://webarchive.di.uminho.pt/haslab.uminho.pt/cbm/files/itc.pdf',''),
('dynamicdatalog24','recent-incremental','Incremental Evaluation of Dynamic Datalog Programs as a Higher-order DBSP Program','Bruno Rucy Carneiro Alves de Lima; Merlin Kramer; Kalmer Apinis; Kristopher Micinski',2024,'Datalog 2.0 2024, CEUR-WS 3801','','https://ceur-ws.org/Vol-3801/paper1.pdf',''),
('kairos24','recent-provenance','KAIROS: Practical Intrusion Detection and Investigation using Whole-system Provenance','Zijun Cheng; Qiujian Lv; Jinyuan Liang; Yan Wang; Degang Sun; Thomas Pasquier; Xueyuan Han',2024,'IEEE Symposium on Security and Privacy 2024','','https://tfjmp.org/publications/2024-sp.pdf',''),
('egwalker25','recent-convergence','Collaborative Text Editing with Eg-walker: Better, Faster, Smaller','Joseph Gentle; Martin Kleppmann',2025,'EuroSys 2025; acquired arXiv:2409.14252v1 preprint (2024)','10.1145/3689031.3696076','https://arxiv.org/pdf/2409.14252',''),
('keyedwatermarks25','recent-streams','Keyed Watermarks: A Fine-grained Watermark Generation for Apache Flink','Tawfik Yasser; Tamer Arafa; Mohamed ElHelw; Ahmed Awad',2025,'Future Generation Computer Systems 169','10.1016/j.future.2025.107796','https://www.sciencedirect.com/science/article/pii/S0167739X25000913','reference-only'),
('prohunter26','recent-provenance','ProHunter: A Comprehensive APT Hunting System Based on Whole-System Provenance','Xuebo Qiu; Mingqi Lv; Yimei Zhang; Tiantian Zhu; Tieming Chen',2026,'arXiv:2603.19658 preprint','10.48550/arXiv.2603.19658','https://arxiv.org/pdf/2603.19658',''),
('provics26','recent-provenance','ProvICS: A Provenance-based Intrusion Detection for Industrial Control Systems','Md Neyamul Islam Shibbir; Deepak K. Tosh',2026,'arXiv:2607.05989 preprint','10.48550/arXiv.2607.05989','https://arxiv.org/pdf/2607.05989',''),
('fixpoint26','recent-incremental','Fixing the Fixpoint: A Formal Theory of Convergence Detection for Incremental Recursive Computation','Chengxi Yang; Tej Chajed; Thomas Reps',2026,'arXiv:2610.00530v1 preprint, submitted 30 September 2026','10.48550/arXiv.2610.00530','https://arxiv.org/pdf/2610.00530v1',''),
]
REFS = [
('timely-dataflow','TimelyDataflow/timely-dataflow','MIT','progress frontiers and capabilities'),
('differential-dataflow','TimelyDataflow/differential-dataflow','MIT','weighted changes and trace compaction'),
('automerge','automerge/automerge','MIT','change graph and convergent document state'),
('flink','apache/flink','Apache-2.0','checkpoint barriers and persisted state'),
('beam','apache/beam','Apache-2.0','watermark and event-time processing interfaces'),
('xtdb','xtdb/xtdb','MPL-2.0','bitemporal transaction and assertion state'),
('gprom','IITDBGroup/gprom','Apache-2.0 (verify LICENSE)','query-rewrite provenance'),
('libprovenance','CamFlow/libprovenance','GPL-2.0 (LICENSE)','whole-system provenance relay consumption'),
]

def sha(path):
    h=hashlib.sha256()
    with path.open('rb') as f:
        for block in iter(lambda:f.read(1024*1024),b''):h.update(block)
    return h.hexdigest()

def get(url, dest):
    if dest.exists(): return
    req=urllib.request.Request(url,headers={'User-Agent':'Deleuze research acquisition; primary publications'})
    with urllib.request.urlopen(req,timeout=60) as r:
        data=r.read()
    if dest.suffix=='.pdf' and not data.startswith(b'%PDF'):
        raise ValueError('not a PDF: '+url)
    dest.parent.mkdir(parents=True,exist_ok=True)
    dest.write_bytes(data)

def table(path, headers, rows):
    with path.open('w') as f:
        w=csv.writer(f,delimiter='\t',lineterminator='\n');w.writerow(headers);w.writerows(rows)

def paper(row, old):
    ident,topic,title,authors,year,venue,doi,url,reuse=row
    dest=ROOT/reuse if reuse and reuse!='reference-only' else ARCH/'papers'/(ident+'.pdf')
    status=old.get(ident,{}).get('reading_status','catalogued')
    if reuse=='reference-only':return [ident,topic,title,authors,year,venue,doi,url,'','','referenced-online',DATE,status,'']
    try:
        get(url,dest)
        txt=SRC/'paper-text'/(ident+'.txt');txt.parent.mkdir(parents=True,exist_ok=True)
        subprocess.run(['pdftotext','-layout',str(dest),str(txt)],check=True,capture_output=True)
        print('paper',ident,dest.stat().st_size,flush=True)
        return [ident,topic,title,authors,year,venue,doi,url,str(dest.relative_to(ROOT)),sha(dest),'existing-corpus' if reuse else 'acquired',DATE,status,'']
    except Exception as e:
        print('paper-failed',ident,str(e),flush=True)
        return [ident,topic,title,authors,year,venue,doi,url,'','','unavailable',DATE,status,str(e)]

def papers():
    old={}
    if (HERE/'bibliography.tsv').exists():
        with (HERE/'bibliography.tsv').open() as f:old={r['id']:r for r in csv.DictReader(f,delimiter='\t')}
    rows=[]
    with ThreadPoolExecutor(max_workers=6) as pool:
        for result in as_completed([pool.submit(paper,r,old) for r in PAPERS]):rows.append(result.result())
    table(HERE/'bibliography.tsv',['id','topic','title','authors','year','venue','doi','primary_url','original','sha256','acquisition_status','acquired_on','reading_status','acquisition_error'],sorted(rows))

def reference(row):
    name,repo,license_hint,purpose=row
    commit=subprocess.run(['git','ls-remote','https://github.com/'+repo+'.git','HEAD'],capture_output=True,text=True,check=True).stdout.split()[0]
    url='https://codeload.github.com/'+repo+'/tar.gz/'+commit
    archive=ARCH/(name+'-'+commit+'.tar.gz')
    get(url,archive)
    dest=SRC/name;dest.mkdir(parents=True,exist_ok=True)
    count=extract(archive,dest)
    licenses=[str(p.relative_to(dest)) for p in dest.iterdir() if p.is_file() and p.name.upper().startswith(('LICENSE','COPYING','NOTICE'))]
    print('ref',name,commit,count,archive.stat().st_size,flush=True)
    return [name,'https://github.com/'+repo,commit,license_hint,'; '.join(licenses),url,str(archive.relative_to(ROOT)),sha(archive),str(dest.relative_to(ROOT)),count,DATE,purpose,'acquired-not-yet-read']

def refs():
    rows=[]
    with ThreadPoolExecutor(max_workers=3) as pool:
        for result in as_completed([pool.submit(reference,r) for r in REFS]):
            try:rows.append(result.result())
            except Exception as e:print('REF FAILED',str(e),flush=True)
    table(HERE/'references.tsv',['id','origin','commit','license_hint','license_files','archive_url','archive','sha256','source','files','acquired_on','purpose','reading_status'],sorted(rows))

def extract(archive,dest):
    excluded={'.git','.DS_Store','__MACOSX','node_modules','target','__pycache__','.venv'}
    count=0
    with tarfile.open(archive) as tar:
        for m in tar:
            p=PurePosixPath(m.name)
            if not m.isfile() or len(p.parts)<2 or '..' in p.parts:continue
            rel=PurePosixPath(*p.parts[1:])
            if any(x in excluded or x.startswith('._') for x in rel.parts):continue
            output=dest/str(rel);output.parent.mkdir(parents=True,exist_ok=True)
            output.write_bytes(tar.extractfile(m).read());count+=1
    return count

def restore():
    """Restore committed revisions from manifests; never resolves moving HEAD."""
    with (HERE/'references.tsv').open() as f:refs=list(csv.DictReader(f,delimiter='\t'))
    for r in refs:
        archive=ROOT/r['archive']
        get(r['archive_url'],archive)
        if sha(archive)!=r['sha256']:raise ValueError('archive hash mismatch: '+r['id'])
        count=extract(archive,ROOT/r['source'])
        if count!=int(r['files']):raise ValueError('archive selection mismatch: '+r['id'])
        print('restored reference',r['id'],count,flush=True)
    with (HERE/'bibliography.tsv').open() as f:papers=list(csv.DictReader(f,delimiter='\t'))
    for r in papers:
        if not r['original']:continue
        original=ROOT/r['original']
        get(r['primary_url'],original)
        if sha(original)!=r['sha256']:raise ValueError('paper hash mismatch: '+r['id'])
    print('Verified',sum(bool(r['original']) for r in papers),'paper originals and',len(refs),'pinned reference archives.')

if __name__=='__main__':
    ARCH.mkdir(parents=True,exist_ok=True);SRC.mkdir(parents=True,exist_ok=True)
    if sys.argv[1:]==['papers']:papers()
    elif sys.argv[1:]==['refs']:refs()
    elif sys.argv[1:]==['restore']:restore()
    else:raise SystemExit('usage: acquire.py papers|refs|restore')
