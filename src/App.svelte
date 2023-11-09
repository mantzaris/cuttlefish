<script lang="ts">
  import { onMount } from 'svelte'
  import MenuBar from './components/MenuBar.svelte'
  import Button, { Label } from '@smui/button'
  import LayoutGrid, { Cell } from '@smui/layout-grid'

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

  <LayoutGrid>
    <Cell span={12} style="height: 25vh;">
      <div class="demo-cell">fdgsdfhg</div>
    </Cell>
    <Cell span={12} style="height: 25vh;">
      <div class="demo-cell">fdhtrtyf445355</div>
    </Cell>
    <Cell span={12} style="height: 25vh;">
      <div class="demo-cell">4545fghfd</div>
    </Cell>
  </LayoutGrid>

  <div id="components">
    <div class="utility">hello1</div>

    <Button touch variant="raised">
      <Label>Raised</Label>
    </Button>

    <h1>cuttlefish</h1>
    <p>Edit <code>src/App.svelte</code> to trigger HMR</p>
    <p>Counter: <code>{count}</code></p>
  </div>
</div>

<style lang="sass">
  .demo-cell
    width: 100%
    background-color: red
  

  #main-page
    display: flex
    flex-direction: column
    height: 100vh
    width: 100vw
    margin: 0
    padding: 0

  #menu
    height: 10vh

  #components
    flex: 1

  .utility
    height: 25vh
    background-color: lightgray  

  h1
    color: #red
  code
    background: #282b33
    padding: 2px 5px
    border-radius: 4px

</style>
