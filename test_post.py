import urllib.request, json

# Start a session
req = urllib.request.Request("http://127.0.0.1:4096/session", data=b"{}", headers={"Content-Type": "application/json"})
try:
    res = urllib.request.urlopen(req)
    session_id = json.loads(res.read())["id"]
    print("Session:", session_id)
except Exception as e:
    print("Failed to create session:", e)
