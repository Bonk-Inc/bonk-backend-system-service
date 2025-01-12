<script setup lang="ts">
import MainLayout from '@/components/layout/MainLayout.vue';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import Checkbox from '@/components/ui/checkbox/Checkbox.vue';
import { FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import { NumberField, NumberFieldContent, NumberFieldInput } from '@/components/ui/number-field';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { useToast } from '@/components/ui/toast';
import { ApiService } from '@/lib/ApiService';
import type { Score, Level, ScoreDTO } from '@/lib/Models';
import type { User } from '@/lib/Models/User';
import { toTypedSchema } from '@vee-validate/zod';
import { useForm } from 'vee-validate';
import { inject, onMounted, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import * as z from 'zod';

const route = useRoute();
const router = useRouter();
const apiService = inject<ApiService>('api');
const { toast } = useToast();

const levels = ref<Level[]>([]);
const users = ref<User[]>([]);

const gameId = route.query['gameId'] as string ?? '';
const scoreId = route.query['scoreId'] ?? '';

const formSchema = toTypedSchema(z.object({
  level_id: z.string().uuid(),
  user_id: z.string().uuid(),
  score: z.number(),
  is_hidden: z.boolean().default(false)
}));

const { handleSubmit, setFieldValue, setValues } = useForm({
  validationSchema: formSchema,
});

const onSubmit = handleSubmit(async (values) => {
  try {
    let description = '';
    if (scoreId) {
      await apiService?.put<Score>(`api/score/${scoreId}`, JSON.stringify(values));
      description = 'Het updaten van de score is gelukt'
    } else {
      await apiService?.post<Score>('api/score', JSON.stringify(values));
      description = 'Het opslaan van de nieuwe score is geluk';
    }

    router.push({ name: 'game_scores', params: { gameId }});
    toast({
      title: 'Gelukt!',
      description
    });
  } catch (e: unknown) {
    toast({
      title: 'Er ging wat fout :(',
      variant: 'destructive',
      description: 'Er is wat fout gegaan tijdens het opslaan van de score'
    });
  }
});

onMounted(async () => {
  try {
    await fetchLevels(gameId);
    await fetchUsers(gameId);

    if (scoreId) {
      const scoreResponse = await apiService?.get<ScoreDTO>(`api/score/${scoreId}`);
      const data = scoreResponse?.data!;
      
      setValues({
        is_hidden: data.is_hidden,
        user_id: data.user_id,
        level_id: data.level_id,
        score: data.score
      })
    }
  } catch (e: unknown) {
    toast({
      title: 'Er ging wat fout :(',
      variant: 'destructive',
      description: 'Er is wat fout gegaan tijdens het ophalen van de data'
    });
  }
});

const fetchLevels = async (gameId: string) => {
  const levelUrl = `api/level/game/${gameId}`;
  const response = await apiService?.get<Level[]>(levelUrl);

  levels.value = response?.data!;
}

const fetchUsers = async (gameId: string) => {
  const userUrl = `api/user/game/${gameId}`;
  const response = await apiService?.get<User[]>(userUrl);

  users.value = response?.data!;
}
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
            <FormField v-slot="{ componentField }" name="user_id">
              <FormItem class="w-4/5">
                <FormLabel>Gebruiker</FormLabel>
                <Select v-bind="componentField">
                  <FormControl>
                    <SelectTrigger>
                      <SelectValue placeholder="Selecteer gebruiker" />
                    </SelectTrigger>
                  </FormControl>
                  <SelectContent>
                    <SelectItem v-for="user in users" :value="user.id" :key="user.id">
                      {{ user.name }}
                    </SelectItem>
                  </SelectContent>
                </Select>

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