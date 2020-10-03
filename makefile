generate_http_server:
	@echo "Creating Exacutable Server"
	@gcc http_server.c -o httpServer

generate_http_client:
	@echo "Creating Exacutable Client"
	@gcc http_client.c -o httpClient

clean: 
	@echo "Cleaning up files"
	@rm server client httpServer