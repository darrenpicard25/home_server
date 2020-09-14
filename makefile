generate:
	@echo "Creating Exacutable"
	@gcc main.c -o server

clean: 
	@echo "Cleaning up files"
	@rm server