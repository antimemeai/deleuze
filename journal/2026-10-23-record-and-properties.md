# Record fields and associated properties

The operator corrected the proposed model: the event has schema-defined record
fields, accompanied by arbitrary property key/value pairs outside that schema.
The prior Event-with-only-properties proposal was an assistant misunderstanding.
Withdrew it and corrected the current design, project instructions and orientation.

A field such as property_hash can belong to a record schema while the property
pairs remain associated data. The example does not require a hash field in every
record, select an algorithm, or dictate inline/separate storage. The actual record
fields and property representation remain to be proposed. Marked the earlier
representation review as superseded while retaining useful ownership/failure
findings. No dependencies, production code or private-source details were added.
