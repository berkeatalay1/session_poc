import requests
import concurrent.futures
import uuid

def send_request(url):
    json_data = {
        "uuid": str(uuid.uuid4())
    }
    response = requests.post(url, json=json_data)
    print(f"Response from {url}: {response.status_code}")

if __name__ == "__main__":
    url = "http://localhost:8080/"

    # Number of concurrent requests
    num_requests = 8

    with concurrent.futures.ThreadPoolExecutor() as executor:
        # Create a list of URL duplicates
        urls = [url] * num_requests
        # Submit the requests concurrently
        executor.map(send_request, urls)
