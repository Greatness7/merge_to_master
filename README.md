# merge_to_master

Get the latest version from the [releases](https://github.com/Greatness7/merge_to_master/releases) page.

A simple command-line tool that lets you merge TES3 plugins into their masters.

```
Merge the contents of a plugin into a master.

Usage: merge_to_master.exe [OPTIONS] <PLUGIN> <MASTER>

Arguments:
  <PLUGIN>  The plugin that will be merged into <MASTER>.
  <MASTER>  The master that <PLUGIN> will be merged into.

Options:
  -r, --remove-deleted                 Remove all objects that are marked as DELETED.
  -o, --overwrite                      Overwrite <MASTER> without creating a backup.
      --preserve-duplicate-references  Preserve duplicate references, if not specified duplicates will be removed.
      --apply-moved-references         Put 'moved references' into their the new cell's reference list. (Experimental)
  -h, --help                           Print help
  -V, --version                        Print version
```
