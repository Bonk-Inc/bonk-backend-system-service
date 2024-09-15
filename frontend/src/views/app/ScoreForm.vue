<script setup lang="ts">
import MainLayout from '@/components/layout/MainLayout.vue';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import Checkbox from '@/components/ui/checkbox/Checkbox.vue';
import { FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import { NumberField, NumberFieldContent, NumberFieldInput } from '@/components/ui/number-field';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { ApiService } from '@/lib/ApiService';
import type { Score, Level } from '@/lib/Models';
import { toTypedSchema } from '@vee-validate/zod';
import { useForm } from 'vee-validate';
import { inject, onMounted, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import * as z from 'zod';

const route = useRoute();
const router = useRouter();
const apiService = inject<ApiService>('api');
const levels = ref<Level[]>([]);
const gameId = route.query['gameId'] ?? '';

const formSchema = toTypedSchema(z.object({
  level_id: z.string().uuid(),
  username: z.string().max(50),
  score: z.number(),
  is_hidden: z.boolean().default(false)
}));

const { handleSubmit, setFieldValue } = useForm({ validationSchema: formSchema });
const onSubmit = handleSubmit(async (values) => {
  try {
    const response = await apiService?.post<Score>('api/score/', JSON.stringify(values));
    router.push({ name: 'game_scores', params: { gameId: gameId }})
  } catch (e: any) {

  }
});

onMounted(async () => {
  try {
    const url = gameId ? `api/level/game/${gameId}/` : 'api/level/';
    const response = await apiService?.get<Level[]>(url);
    levels.value = response?.data!;
  } catch (e: any) {

  }
});
</script>

<template>
  <MainLayout>
    <Card class="w-1/2 container my-4">
      <CardHeader>
        <CardTitle>Score details</CardTitle>
      </CardHeader>
      <CardContent>
        <form @submit="onSubmit">
          <FormField v-slot="{ componentField }" name="level_id">
            <FormItem>
              <FormLabel>Level</FormLabel>
              <Select v-bind="componentField">
                <FormControl>
                  <SelectTrigger>
                    <SelectValue placeholder="Selecteer level" />
                  </SelectTrigger>
                </FormControl>
                <SelectContent>
                  <SelectItem v-for="level in levels" :value="level.id" :key="level.id">
                    {{ level.name }}
                  </SelectItem>
                </SelectContent>
              </Select>

              <FormMessage />
            </FormItem>
          </FormField>
          <div class="flex items-center mt-4 w-full gap-4">
            <FormField v-slot="{ componentField }" name="username">
              <FormItem class="w-4/5">
                <FormLabel>Gebruikersnaam</FormLabel>
                <FormControl>
                  <Input type="text" v-bind="componentField" />
                </FormControl>

                <FormMessage />
              </FormItem>
            </FormField>
            <FormField v-slot="{ value }" name="score">
              <FormItem class="w-1/5">
                <FormLabel>Score</FormLabel>
                <NumberField :min="0" :model-value="value" @update:model-value="(v) => setFieldValue('score', v ?? 0)">
                  <NumberFieldContent>
                    <FormControl>
                      <NumberFieldInput />
                    </FormControl>
                  </NumberFieldContent>
                </NumberField>

                <FormMessage />
              </FormItem>
            </FormField>
          </div>
          <FormField v-slot="{ value, handleChange }" name="is_hidden">
            <FormItem class="mt-4 flex flex-row items-start gap-x-3 space-y-0">
              <FormControl>
                <Checkbox :checked="value" @update:checked="handleChange" />
              </FormControl>

              <div class="space-y-1 leading-none">
                <FormLabel>Verborgen</FormLabel>
                <FormMessage />
              </div>
            </FormItem>
          </FormField>

          <div class="mt-8 flex justify-end">
            <Button>
              Opslaan
            </Button>
          </div>
        </form>
      </CardContent>
    </Card>
  </MainLayout>
</template>