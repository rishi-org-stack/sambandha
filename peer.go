package main

import (
	"bufio"
	"fmt"
	"net"
	"os"
)

func main() {
	// Connect to the server
	conn, err := net.Dial("tcp", "127.0.0.1:7878")
	if err != nil {
		fmt.Println("Error connecting:", err)
		return
	}
	defer conn.Close()
	conn.Write([]byte("search\r\nHeader=user_id:okjhaid01;timestamp:2929221\r\npeer_id:peerdidtest01;"))
	// Read input from the user
	reader := bufio.NewReader(os.Stdin)
	message, _ := reader.ReadString('\n')

	// Send the message to the server
	fmt.Fprintf(conn, message)

	// Receive response from the server
	response, err := bufio.NewReader(conn).ReadString('\n')
	if err != nil {
		fmt.Println("Error receiving:", err)
		return
	}
	fmt.Println("Server response:", response)
}
