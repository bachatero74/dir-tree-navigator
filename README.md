# dir-tree-navigator
Terminal tool for navigating the Linux file system

Add this function to your .bashrc:

nav() {
	{your_path}/navigator $1
	
	local exitcode=$?
	    
	if [ $exitcode -eq 0 ]; then
		cd "$(cat /tmp/navigator.dir)"
		rm /tmp/navigator.dir
	fi
}
