import streamlit as st
import os
import json

@st.experimental_singleton
def load_data():
    result = {}
    for f in filter(lambda x: ".json" in x, os.listdir(".")):
        result[f] = []
        with(open(f, 'rt')) as json_reader:
            for k,v in json.loads(json_reader.read()).items():
                result[f].append({"tool": k, "count": v})
    return result

data = load_data()
file = st.selectbox("files", data.keys())
subset = data[file]

st.bar_chart(subset, x="tool", y="count")
