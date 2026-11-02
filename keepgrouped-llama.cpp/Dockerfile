FROM ghcr.io/ggml-org/llama.cpp:server-vulkan

ENV RADV_PERFTEST=nogttspill

EXPOSE 9931

RUN apt update && apt install wget

RUN wget -nv https://huggingface.co/unsloth/gemma-4-E2B-it-qat-GGUF/resolve/main/gemma-4-E2B-it-qat-UD-Q4_K_XL.gguf
RUN wget -nv https://huggingface.co/unsloth/gemma-4-E2B-it-qat-GGUF/resolve/main/mmproj-F16.gguf

ENTRYPOINT [ "/app/llama-server", "-m", "gemma-4-E2B-it-qat-UD-Q4_K_XL.gguf", "--mmproj", "mmproj-F16.gguf", "-c", "8192", "-ngl", "99", "--port", "9931", "--sleep-idle-seconds", "30", "-fa", "on", "--temp", "1.0", "--top-p", "0.95", "--top-k", "64", "--jinja" ]