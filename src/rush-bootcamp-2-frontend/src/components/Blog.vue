<template>
    <h2 class="text-blue-600">Wpisy</h2>
    <div class="w-100 flex flex-row-reverse">
        <button @click="pobierzWpisy" class="bg-blue-600 rounded text-white p-4">POBIERZ</button>
    </div>
    <div class="grid mx-6 gap-4 my-4">
        <div v-for="wpis in wpisy" class="drop-shadow-xl bg-stone-300 p-4">
            <p>{{ wpis }}</p>
        </div>
    </div>
    <input v-model="nowyBlog" type="text" class="border-1 border-blue-600 p-4">
    <button @click="dodajWpis" class="bg-blue-600 rounded text-white p-4">DODAJ</button>
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
        },

        async mounted() {
            this.pobierzWpisy();
        }

    }
</script>