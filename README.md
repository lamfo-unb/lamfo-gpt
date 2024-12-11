# LAMFO GPT
**Developed by:** Carlos David Castro de Souza Neto

LAMFO GPT is an AI-powered assistant specialized in the LAMFO (Machine Learning Laboratory in Finance and Organizations). This is an ambitious project to test the use of personalized assistants to answer specific questions about an organization.

My idea is to expand the knowledge of LAMFO GPT to gather information from our website, videos, and database. In the future, it could help in other areas such as Law, Economics, News, and many other fields depending on your imagination.

## Dependencies

* Rust
* Docker

## Run all in Docker

1. Upload the API image
    ```bash
    docker build -f Dockerfile -t dauid64/lamfo-gpt .
    ```
2. Run the docker-compose
    ```bash
    docker-compose up -d
    ```

## Run development

1. Run the docker-compose-dev
    ```bash
        docker-compose -f docker-compose-dev.yml up -d
    ```
2. Start the application
    ```bash
        cargo watch -q -c -w src -w .cargo -x "run"
    ```
    or
     ```bash
        cargo run
    ```

## Endpoints

URL   | Method
--------- | ------
http://localhost:3000/api/lamfo-gpt/chat | POST

Body
```json
{ 
    "content": "Your message"
}
 ```

## Warnings

NEVER push anything to the main branch. Automatically, when you push something to it, an action for production will be triggered. So always use another branch for testing and experiments.
