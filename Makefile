README.md: README.master.md
	perl -ne \
	    'if(/^\$$\$$/){s/^\$$//;print;s/^\$$\s*//;s/\n$$//;system $$_}elsif(/^\$$/){s/^\$$\s*//;s/\n$$//;system $$_}else{print}' \
		$< >$@ 2>/dev/null
