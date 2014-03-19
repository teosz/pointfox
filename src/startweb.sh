echo 'Start server on http://localhost:'$1
while true;
 do
 { 
	echo -e 'HTTP/1.1 200 OK\r\n';
	cat ../.web/templates/index.html; 
	
} | nc -l "$1";done
