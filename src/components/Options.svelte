<script>
  // @ts-nocheck

  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  export let timerStarted = false;

  let timer = "";
  let isInvalid = false;

  function validateInput() {
    // Regular expression to validate HH:MM:SS format
    const regex = /^([0-1]\d|2[0-3]):([0-5]\d):([0-5]\d)$/;
    isInvalid = !regex.test(timer);
  }

  function handleKeyDown(e) {
    if (
      !e.key &&
      /^[0-9]$/i.test(e.target.value[e.target.value.length - 1]) &&
      e.target.value.length <= 8
    ) {
      if (e.target.value.length == 2) {
        timer = e.target.value + ":";
      } else if (e.target.value.length == 5) {
        timer = e.target.value + ":";
      } else {
        timer = e.target.value;
      }
    } else if (e.key === "Backspace") {
      if (e.target.value.length == 6) {
        timer = e.target.value.slice(0, 5);
      } else if (e.target.value.length == 3) {
        timer = e.target.value.slice(0, 2);
      } else {
        timer = e.target.value.slice(0, e.target.value.length);
      }
    }
  }
</script>

<div>
  <input
    type="text"
    placeholder="HH:MM:SS"
    value={timer}
    on:input={(e) => {
      isInvalid = false;
      handleKeyDown(e);
    }}
    on:keydown={(e) => {
      if (e.key === "Backspace") handleKeyDown(e);
    }}
    class:is-invalid={isInvalid}
    autocomplete="off"
  />
  {#if timerStarted}
    <button
      on:click={() => {
        isInvalid = false;
        dispatch("clear");
      }}>Stop</button
    >
  {:else}
    <button
      on:click={() => {
        validateInput();
        if (!isInvalid) {
          dispatch("start", { timer });
        }
      }}>Start</button
    >
  {/if}
</div>

<style>
  ::placeholder {
    text-align: center;
  }

  div {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 1em;
  }

  input,
  button {
    border-radius: 8px;
    border: 2px solid transparent;
    padding: 0.4em 1.1em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #ffffff;
    background-color: #0f0f0f98;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  input {
    width: 150px;
    text-align: center;
  }

  button {
    cursor: pointer;
  }

  input:hover,
  button:hover {
    border-color: #396cd8;
  }

  button:active {
    border-color: #396cd8;
    background-color: #0f0f0f69;
  }

  input,
  button {
    outline: none;
  }

  input.is-invalid {
    border-color: rgba(220, 20, 60, 0.658);
  }
</style>
