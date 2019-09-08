README.md: README.master.md src/lib.rs src/bin/xpg.rs
	PATH="$(shell pwd)/target/debug:${PATH}" \
	perl -ne \
	    'if(/^\$$\$$/){s/^\$$//;print;s/^\$$\s*//;s/\n$$//;system $$_}elsif(/^\$$/){s/^\$$\s*//;s/\n$$//;system $$_}else{print}' \
		$< >$@ 2>/dev/null
