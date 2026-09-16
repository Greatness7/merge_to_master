# merge_to_master

Get the latest version from the [releases](https://github.com/Greatness7/merge_to_master/releases) page.

A simple command-line tool that lets you merge TES3 plugins into their masters.

```
Merge the contents of a plugin into a master.

Usage: merge_to_master.exe [OPTIONS] [PLUGIN] [MASTER]

Arguments:
  [PLUGIN]  The plugin that will be merged into <MASTER>.
  [MASTER]  The master that <PLUGIN> will be merged into.

Options:
      --list <LIST>                    A text file listing plugins to merge, in load order (one per line).
      --output <OUTPUT>                The path to save the merged <LIST> to.
  -r, --remove-deleted                 Remove all objects that are marked as DELETED.
  -o, --overwrite                      Overwrite the output file without creating a backup.
      --preserve-duplicate-references  Preserve duplicate references, if not specified duplicates will be removed.
      --apply-moved-references         Put 'moved references' into the new cell's reference list. (Experimental)
  -h, --help                           Print help
  -V, --version                        Print version
```

## Merging a plugin into its master

```
merge_to_master.exe "My Plugin.esp" "My Master.esm"
```

The merged result is saved over `<MASTER>`. A backup of the original is created unless `--overwrite` is used.

## Merging an entire load order

```
merge_to_master.exe --list=loadorder.txt --output=merged.esm
```

`<LIST>` is a text file naming the plugins to merge, one per line, in load order.

```
Morrowind.esm
Tribunal.esm
Bloodmoon.esm
```

The merged result is saved to `<OUTPUT>`. If that file already exists, a backup is created unless `--overwrite` is used.
