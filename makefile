generate:
	@echo "Creating Exacutable"
	@gcc main.c sum.c -o main

clean: 
	@echo "Cleaning up files"
	@rm main