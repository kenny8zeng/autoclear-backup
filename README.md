# AutoClear

Traverse all files in the specified directory under the Linux environment, determine whether to keep them based on the file modification time, and delete the files that do not need to be kept. Only keep the latest copies from one day ago, one week ago, one month ago, one year ago, and two years ago.

## Usage

```
autoclear [-p|--prefix <prefix>] [-t|--test] [directory]
```

- `directory` is the directory to be cleared. If not specified, the current directory is used.
- `-p|--prefix <prefix>` is an optional argument that specifies a prefix to filter the files to be deleted. If not specified, all files will be deleted.
- `-t|--test` is an optional argument that runs the program in test mode, which only prints the files that would be deleted without actually deleting them.

## Example

```shell
autoclear -p "backup_" /path/to/directory
```

This command will delete all files in `/path/to/directory` that have the prefix `backup_` in their name. However, the latest copies from one day ago, one week ago, one month ago, one year ago, and two years ago are retained.
