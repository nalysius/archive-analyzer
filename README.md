# Archive Analyzer

Archive Analyzer is a program used to parse and analyze archives. It's in early
stage and is developed only to learn the internals of archive formats.

There is no external dependencies, and the parsing is done manually.
The code is commented, and there are seveal "TODO" here and there, especially
in the file src/reader.rs (the parser).

## Supported archive formats

Below the list of the supported formats and the ones that could be supported
in the future.

- [x] ZIP
- [ ] TAR
- [ ] RAR
- [ ] 7Z

### ZIP

Some parts of the ZIP specs have been ignored as of now, but could be supported
in the future.

- [x] Local File Header
- [x] Digital Signature
- [x] Central File Directory
- [ ] Encrypted files
- [ ] Executable ZIP

## Evolution

The structure make this program open to evolution. in the future it would be nice
to have "exporters", structs that exports the data of an archive instead of just
writing it to STDOUT. A CSV exporter and a STDOUT exporter should be enough.


Moreover, once the parsing is done, several actions are possible:
- Removing a file from the Central Directory (ZIP-specific, but would it be
extracted if unzipped?)
- Updating specific values to test the behaviour of other softwares