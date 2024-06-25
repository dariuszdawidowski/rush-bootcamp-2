<template>
    <button @click="pobierzWpisy">POBIERZ</button>
    <div>Hello Blog!</div>
    {{ wpisy }}
</template>

<script>

    import { rush_bootcamp_2_backend } from 'declarations/rush-bootcamp-2-backend/index';

    export default {

        data() {
            return {
                wpisy: []
            };
        },

        methods: {
            async pobierzWpisy() {
                this.wpisy = await rush_bootcamp_2_backend.odczytaj_wpisy();
            }
        }

    }
</script>