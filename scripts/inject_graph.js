const fs = require("fs");
const path = require("path");

const targetPaths = [
  path.join(__dirname, "../graphify-out/graph.html"),
  path.join(__dirname, "../docs/graph.html"),
];

const injectedScript = `
        <style>
            #graph   { width: 100% !important; flex: 1 !important; background: radial-gradient(circle at center, #131320 0%, #0a0a10 100%) !important; }
            #graph canvas { filter: drop-shadow(0 0 12px rgba(99,102,241,0.12)); }
            body     { overflow: hidden !important; margin: 0; background: #0a0a10; }
        </style>
        <script>
            // Expose on window so parent iframe can access
            window.network = network;

            network.once('stabilizationIterationsDone', () => {
                network.setOptions({ physics: { enabled: false } });
                window.networkReady = true;

                // ── Cinematic node-touring animation ─────────────────────────────────────
                const hubNodes = RAW_NODES.slice().sort((a, b) => b.degree - a.degree).slice(0, 40);

                let tourStep = 0;
                let tourTimer = null;
                let driftTimer = null;
                let overviewCounter = 0;

                function rand(a, b) { return a + Math.random() * (b - a); }

                function runTour() {
                    clearTimeout(tourTimer);
                    clearTimeout(driftTimer);

                    if (window.apexPaused) {
                        tourTimer = setTimeout(runTour, 500);
                        return;
                    }

                    const doOverview = (overviewCounter % 5 === 0);
                    overviewCounter++;

                    if (doOverview) {
                        network.unselectAll();
                        network.moveTo({
                            position: { x: rand(-100, 100), y: rand(-80, 80) },
                            scale: rand(0.25, 0.35),
                            animation: { duration: Math.round(rand(4000, 5000)), easingFunction: 'easeInOutQuart' }
                        });
                        tourTimer = setTimeout(runTour, rand(5000, 6000));
                        return;
                    }

                    const node = hubNodes[tourStep % hubNodes.length];
                    tourStep++;

                    const positions = network.getPositions([node.id]);
                    const pos = positions[node.id];
                    if (!pos) { tourTimer = setTimeout(runTour, 400); return; }

                    network.unselectAll();
                    network.selectNodes([node.id]);

                    const nodeScale = rand(1.6, 2.2);
                    const travelTime = Math.round(rand(3500, 4800));

                    network.moveTo({
                        position: { x: pos.x, y: pos.y },
                        scale: nodeScale,
                        animation: { duration: travelTime, easingFunction: 'easeInOutQuart' }
                    });

                    const holdTime = Math.round(rand(2000, 3000));
                    
                    // Cinematic drift: slowly pan and zoom slightly while holding on the node
                    driftTimer = setTimeout(() => {
                        if (window.apexPaused) return;
                        network.moveTo({
                            position: { x: pos.x + rand(-20, 20), y: pos.y + rand(-20, 20) },
                            scale: nodeScale * rand(1.02, 1.08),
                            animation: { duration: holdTime, easingFunction: 'linear' }
                        });
                    }, travelTime + 50);

                    tourTimer = setTimeout(runTour, travelTime + holdTime);
                }

                tourTimer = setTimeout(runTour, 1200);

                window.stopTour = () => { 
                    clearTimeout(tourTimer); 
                    clearTimeout(driftTimer); 
                    network.unselectAll(); 
                };
                window.startTour = () => { 
                    clearTimeout(tourTimer); 
                    clearTimeout(driftTimer); 
                    tourTimer = setTimeout(runTour, 600); 
                };
            });
        </script>
`;

for (const file of targetPaths) {
  if (fs.existsSync(file)) {
    let html = fs.readFileSync(file, "utf8");
    if (!html.includes("window.networkReady")) {
      // Strip out the bulky sidebar HTML, replacing it with barebones hidden elements
      // so the original script's event listeners don't throw errors.
      const mockSidebar = `<div id="sidebar" style="display: none;">
  <input id="search">
  <div id="search-results"></div>
  <div id="info-content"></div>
</div>
<script>`;
      html = html.replace(/<div id="sidebar">[\s\S]*?<\/div>\s*<script>/, mockSidebar);

      html = html.replace("</body>", injectedScript + "\n</body>");
      fs.writeFileSync(file, html);
      console.log("Successfully injected animation and stripped sidebar in " + file);
    } else {
      console.log(file + " already injected.");
    }
  }
}
