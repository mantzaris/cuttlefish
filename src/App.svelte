<script lang="ts">
  import { onMount } from 'svelte'
  import MenuBar from './components/MenuBar.svelte'
  import Button, { Label } from '@smui/button'
  import LayoutGrid, { Cell } from '@smui/layout-grid'


</script>

<body>

  <div id="menu">
    <MenuBar />
  </div>

  <LayoutGrid>
    <Cell span={12}>
      <div class="cell-component">Content of the first row</div>
    </Cell>
    <Cell span={12} >
      <div class="cell-component">Content of the 2 row</div>
    </Cell>
    <Cell span={12} >
      <div class="cell-component">Content of the 3 row</div>
    </Cell>
    <Cell span={12} >
      <div class="cell-component">Content of the 4 row</div>
    </Cell>
    <Cell span={12} >
      <div class="cell-component">Content of the 5 row</div>
    </Cell>
  </LayoutGrid>

</body>

<style lang="sass">
  
  body
    overflow: hidden
    height: 100%
    margin: 0
    padding: 0

  #menu
    height: 10vh
  
  .cell-component
    height: 25vh
    margin-top: 5px
    background-color: brown


</style>
