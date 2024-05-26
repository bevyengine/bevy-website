- Changed SystemId fields from tuple struct to a normal struct
If you want to access the entity field, you should use `SystemId::entity` instead of `SystemId::0`
