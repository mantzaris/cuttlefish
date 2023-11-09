<script lang="ts">
  import { onMount } from 'svelte'
  import MenuBar from './components/MenuBar.svelte'

  let count = 0
  onMount(() => {
    const interval = setInterval(() => count++, 1000)
    return () => clearInterval(interval)
  })
</script>

<div id="main-page">
  <div id="menu">
    <MenuBar />
  </div>

  <div id="components">
    <h1>cuttlefish3</h1>
    <p>Edit <code>src/App.svelte</code> to trigger HMR</p>
    <p>Counter: <code>{count}</code></p>
  </div>
</div>

<style lang="sass">
  :global(body)
    margin: 0
    padding: 0
    font-family: Arial, Helvetica, sans-serif
    font-size: 18px
    background-color: blue
    color: #f2f2f2
    text-align: center
  
  #main-page
    display: flex
    flex-direction: column
    height: 100vh
    margin: 0
    padding: 0

  #menu
    height: 10%

  #components
    flex: 1

  h1
    color: #red
  code
    background: #282b33
    padding: 2px 5px
    border-radius: 4px
</style>
