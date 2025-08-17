create:
	mkdir $(p)
	touch $(p)/main.rs
	touch $(p)/makefile
	touch $(p)/.gitignore
	echo main >> $(p)/.gitignore
	echo run:\\n\\t "rustc main.rs && echo "Running program..." && ./main" >> $(p)/makefile

run:
	rustc $(p)/main.rs && echo "\nRunning program...\n" && ./main