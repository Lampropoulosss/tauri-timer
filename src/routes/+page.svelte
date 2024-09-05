<script>
  // @ts-nocheck

  document.addEventListener("contextmenu", (event) => event.preventDefault());

  import { invoke } from "@tauri-apps/api/tauri";
  import Options from "../components/Options.svelte";
  import { writable } from "svelte/store";
  import { onMount } from "svelte";

  let timer = writable(0);
  let interval;
  let audio;

  let isPaused = false;
  let alarmFinished = false;

  onMount(() => {
    fetch("/audio.mp3")
      .then((response) => response.blob())
      .then((blob) => {
        const url = URL.createObjectURL(blob);
        audio = new Audio(url);
        audio.loop = true;
      });
  });

  function formatTime(totalSeconds) {
    const hours = String(Math.floor(totalSeconds / 3600)).padStart(2, "0");
    const minutes = String(Math.floor((totalSeconds % 3600) / 60)).padStart(
      2,
      "0"
    );
    const seconds = String(totalSeconds % 60).padStart(2, "0");
    return `${hours}:${minutes}:${seconds}`;
  }

  // @ts-ignore
  function startTimer(event) {
    const [hours, minutes, seconds] = event.detail.timer.split(":").map(Number);
    let totalSeconds = hours * 3600 + minutes * 60 + seconds;

    timer.set(totalSeconds);

    if (interval) {
      clearInterval(interval);
    }

    interval = setInterval(() => {
      if (isPaused) return;

      totalSeconds -= 1;
      timer.set(totalSeconds);

      if (totalSeconds <= 0) {
        clearInterval(interval);
        alarmFinished = true;
        timer.set(0);
        invoke("notify");
        audio.play();
      }
    }, 1000);

    return () => clearInterval(interval);
  }

  function clearTimer() {
    clearInterval(interval);
    timer.set(0);
    audio.pause();
    audio.currentTime = 0;
    isPaused = false;
    alarmFinished = false;
  }

  function pause_resume() {
    isPaused = !isPaused;
  }
</script>

<div class="container">
  <div class="timer" class:colored={alarmFinished}>
    {#if $timer > 0}
      {formatTime($timer)}
    {:else}
      00:00:00
    {/if}
  </div>
  <Options
    on:start={startTimer}
    on:clear={clearTimer}
    on:pause-resume={pause_resume}
    {isPaused}
    {alarmFinished}
    timerStarted={$timer > 0 || !audio?.paused}
  />
</div>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #f6f6f6;
    background-color: #2f2f2f;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  :global(*) {
    box-sizing: border-box;
  }

  :global(body) {
    margin: 0;
    padding: 0;
  }

  .container {
    min-height: 100vh;
    display: flex;
    align-items: center;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  .colored {
    color: #00ced1;
  }

  .timer {
    font-size: 3em;
    font-weight: 700;
    margin-bottom: 0.4em;
  }
</style>
