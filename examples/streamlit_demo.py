import streamlit as st
import pandas as pd
import plotly.express as px
from emoji_sentiment import EmojiSentiment

if __name__ == "__main__":
    emoji_sentiment = EmojiSentiment()

    st.title("Emoji Sentiment")
    st.markdown(
        "Dataset: [Emoji sentiment ranking](https://kt.ijs.si/data/Emoji_sentiment_ranking/)")

    df = pd.DataFrame([e.model_dump() for e in emoji_sentiment.all])

    fig = px.scatter(
        data_frame=df,
        x="score",
        y="samples",
        text="char",
        hover_data={"name": True, "short_names": True, "char": False,
                    "score": True, "samples": True},
        render_mode="svg"
    )

    fig.update_traces(
        textposition="middle center",
        mode="text",
        textfont_size=20,
        showlegend=True
    )

    st.plotly_chart(fig, width="stretch")

    st.write("---")
    st.header("Search emoji by short name")

    emoji_search = st.text_input(
        "e.g., 'smile', 'eyes'")
    if (emoji := emoji_sentiment.get(emoji_search)) is not None:
        st.markdown(f"**Emoji**")
        st.write(emoji.char)
        st.markdown(f"**Name**")
        st.markdown(f"`{emoji.name}`")
        st.markdown(f"**Short names**")
        st.write(emoji.short_names)
        st.markdown(f"**Sentiment score**")
        st.write(emoji.score)
        st.markdown(f"**Number of samples**")
        st.write(emoji.samples)
    else:
        st.markdown("No emoji found with that short name.")
