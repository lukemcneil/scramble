<script lang="ts">
	export let text: string;
	export let value = '';
	// $: console.log(value);
</script>

<input bind:value placeholder={text} />

<!-- <p> Hello {name || 'person'}!</p> -->

<style>
	@import '../app.css';
</style>
