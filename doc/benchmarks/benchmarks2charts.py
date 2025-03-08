import os
import json
from copy import deepcopy

import plotly.graph_objects as go

data = {}
for fileName in os.listdir():
    if fileName.endswith(".json"):
        data[fileName] = json.loads(open(fileName, "rb").read())

levels = ["DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL"]
d = deepcopy(data["linux_pybenchmarks.json"])

for size, sizev in d.items():
    for exc, excv in sizev.items():
        for kl, vl in excv.items():
            d3 = {}
            for ll, l in vl.items():
                for k, v in l.items():
                    d3.setdefault(k, {})[ll] = v
            fig = go.Figure(
                data=[
                    go.Bar(
                        name=k,
                        x=x,
                        y=y,
                        text=[round(v, 2) for v in y],
                        textposition="auto",
                    )
                    for k, v, in d3.items()
                    if (x := list(v.keys()))
                    if (y := list(v.values()))
                ]
            )
            # Change the bar mode
            sizeStr = "Short log message" if size == "short" else "Long log message"
            excStr = "No exception" if exc == "noexc" else "Exception"
            klStr = (
                "No logging"
                if kl == "nolog"
                else "File" if kl == "file" else "File rotating"
            )
            fig.update_layout(title_text=f"{sizeStr} - {excStr} - {klStr}")
            fig.update_layout(barmode="group")
            fig.update_traces(
                textfont_size=12,
                textangle=-45,
                textposition="outside",
                cliponaxis=False,
            )
            # fig.update_xaxes(tickangle=45)
            # fig.show()
            print(f"Writing {size}-{exc}-{kl}.svg")
            fig.write_image(f"{size}-{exc}-{kl}.svg")
