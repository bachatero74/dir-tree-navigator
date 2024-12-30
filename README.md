# dir-tree-navigator
Terminal tool for navigating the Linux file system

To allow the navigator change current dir, add this function to your .bashrc:

```bash
nav() {
	your_path/navigator $1
	
	local exitcode=$?
	    
	if [ $exitcode -eq 0 ]; then
		cd "$(cat /tmp/navigator.dir)"
		rm /tmp/navigator.dir
	fi
}
```

You can run the navigator by typing nav. When you finish the app with F10 key, current directory will change to selected one.
Esc quits app without changing current dir.
