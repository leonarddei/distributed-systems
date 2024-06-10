# Distributed Systems Lab
Simple backend application for a todo list

## Installation
1. Clone the repository
```bash
git clone https://github.com/leonarddei/distributed-systems
``` 
2. Navigate to the project directory
```bash
cd distributed-systems
``` 

## Usage
1. Start the application using Docker Compose
```bash
   docker compose up
``` 

## Docker Image
https://hub.docker.com/r/leonarddei/distributed-systems-lab/

## API Endpoints

### GET /
- Description: Returns a hello world message
- Method: `GET`
- Response:
    - Status Code: `200 (OK)`
        - Body: `Hello, World!`

### GET /todos
- Description: Fetches all todo items from the database
- Method: `GET`
- Response:
  - Status Code: `200 (OK)`
    - Body Example: `["Hausaufgaben","Task 2"]`
  - Status Code: `500 (INTERNAL_SERVER_ERROR)`

### GET /todos/:todo
- Description: Fetches one todo item from the database
- Method: `GET`
- Response:
    - Status Code: `200 (OK)`
        - Body Example: `"Hausaufgaben"`
    - Status Code: `404 (NOT_FOUND)`
    - Status Code: `500 (INTERNAL_SERVER_ERROR)`

### POST /todos/:todo
- Description: Inserts a new todo item into the database
- Method: `POST`
- Response:
    - Status Code: `201 (CREATED)`
        - Body Example: `"Hausaufgaben"`
    - Status Code: `500 (INTERNAL_SERVER_ERROR)`

### DELETE /todos/:todo
- Description: Deletes a todo item from the database
- Method: `DELETE`
- Response:
    - Status Code: `200 (OK)`
        - Body Example: `"Hausaufgaben"`
    - Status Code: `500 (INTERNAL_SERVER_ERROR)`
