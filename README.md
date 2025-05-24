# judo  
**Just Do IT!** A no-nonsense TODO tracker for your codebase.

---

## What is judo?

`judo` scans your project files for `// TODO:` and `// FIXME:` comments, assigns each a unique ID, and makes them trackable and manageable from the command line.

All data is stored in a `.judo/` directory.  
Deleting it wipes your tracked TODOs â€” you can rescan, but why risk it?

---

## Commands

### Scan the project  
Search for all TODOs/FIXMEs in code:
```sh
judo scan
```

### Add a manual TODO  
Useful for general project reminders:
```sh
judo add "Rewrite the config parser"
```

### List all TODOs  
View all tracked items with their locations and UIDs:
```sh
judo list
```

### Mark a TODO as done  
Removes it from the list:
```sh
judo ihavedone <UID>
```

### Get motivated  
Displays a motivational quote to kickstart your focus:
```sh
judo motivate
```

**Bonus**: Add it to your shell startup for daily motivation:
```sh
echo 'judo motivate' >> ~/.bashrc
```

---

## Notes

- `.judo/` holds the cache and metadata. Keep it safe.
- Re-scanning is possible but slower on large projects.
- Fast. Lightweight. Parallel.
