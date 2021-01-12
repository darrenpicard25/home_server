generate:
	@echo "Creating Exacutable"
	@gcc ./src/app.c -o server

debug:
	@echo "Debug Creation"
	@gcc -g ./src/app.c -o server

clean: server
	@echo "Cleaning up Files"
	rm server
