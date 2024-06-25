<template>
    <button @click="pobierzWpisy">POBIERZ</button>
    <div>Hello Blog!</div>
    {{ wpisy }}
    <input v-model="nowyBlog" type="text">
    <button @click="dodajWpis">DODAJ</button>
</template>

<script>

    import { rush_bootcamp_2_backend } from 'declarations/rush-bootcamp-2-backend/index';

    export default {

        data() {
            return {
                wpisy: [],
                nowyBlog: ''
            };
        },

        methods: {
            async dodajWpis() {
                await rush_bootcamp_2_backend.dodaj_wpis(this.nowyBlog);
                this.pobierzWpisy();
            },
            async pobierzWpisy() {
                this.wpisy = await rush_bootcamp_2_backend.odczytaj_wpisy();
            }
        }

    }
</script>