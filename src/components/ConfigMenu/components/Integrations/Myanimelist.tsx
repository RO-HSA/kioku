import { TextField, Typography } from '@mui/material';
import { useForm } from 'react-hook-form';

import Button from '@/components/ui/Button';
import useMyanimelistCallback from '@/hooks/integrations/useMyanimelistCallback';
import { MyAnimeListService } from '@/services/backend/MyAnimeList';
import { useMyAnimeListStore } from '@/stores/config/providers/myanimelist';

type FormData = {
  username: string;
};

const MyanimelistForm = () => {
  const username = useMyAnimeListStore((state) => state.username);
  const setUsername = useMyAnimeListStore((state) => state.setUsername);
  const isAuthenticated = useMyAnimeListStore((state) => state.isAuthenticated);
  const isAuthenticating = useMyAnimeListStore(
    (state) => state.isAuthenticating
  );
  const listData = useMyAnimeListStore((state) => state.listData);
  const setIsAuthenticating = useMyAnimeListStore(
    (state) => state.setIsAuthenticating
  );
  console.log({ listData: JSON.parse(listData) });

  const {
    register,
    handleSubmit,
    formState: { errors }
  } = useForm<FormData>();

  const onSubmit = async () => {
    setIsAuthenticating(true);
    await MyAnimeListService.authorize();
  };

  useMyanimelistCallback();

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="flex flex-col gap-2">
      <Typography variant="h6">MyAnimeList</Typography>

      <div className="flex gap-1">
        <TextField
          error={!!errors.username}
          helperText={errors.username ? errors.username.message : ''}
          {...register('username', {
            required: 'Username is required'
          })}
          label="Username"
          size="small"
          variant="outlined"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
        />

        <div>
          <Button
            variant="primary"
            size="medium"
            type="submit"
            isDisabled={isAuthenticating && !isAuthenticated}
            isLoading={isAuthenticating && !isAuthenticated}>
            {!isAuthenticated ? 'Authorize' : 'Re-authorize'}
          </Button>
        </div>
      </div>
    </form>
  );
};

export default MyanimelistForm;
