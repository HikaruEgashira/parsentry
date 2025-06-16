import requests
def handle_request(request):
    query = request.get("query")
    conn.execute(query)
