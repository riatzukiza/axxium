#!/usr/bin/env python3
"""Simple MusicGen server for text-to-audio generation."""

import asyncio
import base64
import json
import os
import io
import wave
from aiohttp import web

try:
    import torch
    from transformers import AutoProcessor, MusicgenForConditionalGeneration
    MODEL_LOADED = False
    model = None
    processor = None
    device = "cuda" if torch.cuda.is_available() else "cpu"
except ImportError:
    MODEL_LOADED = False
    model = None
    processor = None
    device = "cpu"

async def load_model():
    """Load the MusicGen model."""
    global model, processor, MODEL_LOADED
    if MODEL_LOADED:
        return
    
    print(f"Loading MusicGen model on {device}...")
    processor = AutoProcessor.from_pretrained("facebook/musicgen-small")
    model = MusicgenForConditionalGeneration.from_pretrained("facebook/musicgen-small")
    model = model.to(device)
    MODEL_LOADED = True
    print("Model loaded!")

async def generate_music(request):
    """Generate music from text prompt."""
    try:
        data = await request.json()
        prompt = data.get("prompt", "")
        duration = data.get("duration", 5)
        
        if not prompt:
            return web.json_response({"error": "prompt required"}, status=400)
        
        await load_model()
        
        # Generate
        inputs = processor(
            text=[prompt],
            padding=True,
            return_tensors="pt",
        ).to(device)
        
        max_new_tokens = int(duration * 50)  # 50 tokens per second
        
        with torch.no_grad():
            audio_values = model.generate(**inputs, max_new_tokens=max_new_tokens)
        
        # Convert to wav
        audio_data = audio_values[0, 0].cpu().numpy()
        sampling_rate = model.config.audio_encoder.sampling_rate
        
        # Convert to bytes
        audio_bytes = (audio_data * 32767).astype("int16").tobytes()
        
        # Create WAV
        wav_buffer = io.BytesIO()
        with wave.open(wav_buffer, "wb") as wav_file:
            wav_file.setnchannels(1)
            wav_file.setsampwidth(2)
            wav_file.setframerate(sampling_rate)
            wav_file.writeframes(audio_bytes)
        
        wav_bytes = wav_buffer.getvalue()
        
        # Encode as base64
        audio_b64 = base64.b64encode(wav_bytes).decode("utf-8")
        
        return web.json_response({
            "status": "success",
            "data": {
                "audio": audio_b64,
                "format": "wav",
                "duration": duration,
                "sample_rate": sampling_rate
            }
        })
        
    except Exception as e:
        return web.json_response({"error": str(e)}, status=500)

async def health(request):
    """Health check endpoint."""
    return web.json_response({
        "status": "ok",
        "model_loaded": MODEL_LOADED,
        "device": device
    })

async def unload_model():
    """Unload model to free GPU memory."""
    global model, processor, MODEL_LOADED
    if model is not None:
        del model
        model = None
    if processor is not None:
        del processor
        processor = None
    if torch.cuda.is_available():
        torch.cuda.empty_cache()
    MODEL_LOADED = False
    print("Model unloaded")

async def main():
    app = web.Application()
    app.router.add_post("/generate", generate_music)
    app.router.add_get("/health", health)
    app.router.add_post("/unload", lambda req: unload_model() or web.json_response({"status": "unloaded"}))
    
    runner = web.AppRunner(app)
    await runner.setup()
    site = web.TCPSite(runner, "0.0.0.0", 8080)
    await site.start()
    
    print("MusicGen server started on port 8080")
    
    while True:
        await asyncio.sleep(3600)

if __name__ == "__main__":
    asyncio.run(main())
