const CACHE = "namer-v1";
self.addEventListener("install", () => { self.skipWaiting(); });
self.addEventListener("activate", (e) => { e.waitUntil(self.clients.claim()); });
self.addEventListener("fetch", (e) => {
  const req = e.request;
  if (req.method !== "GET") return;
  e.respondWith(
    fetch(req).then((r) => {
      if (r && r.status === 200 && r.type === "basic") {
        const c = r.clone();
        caches.open(CACHE).then((ca) => ca.put(req, c)).catch(() => {});
      }
      return r;
    }).catch(() => caches.match(req))
  );
});
