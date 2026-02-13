import { TextField } from '@mui/material';
import { useForm } from 'react-hook-form';

import Button from '@/components/ui/Button';
import useMyanimelistCallback from '@/hooks/integrations/useMyanimelistCallback';
import { MyAnimeListService } from '@/services/backend/MyAnimeList';
import { useMyAnimeListStore } from '@/stores/providers/myanimelist';
import { openUrl } from '@tauri-apps/plugin-opener';

type FormData = {
  username: string;
};

const MyanimelistForm = () => {
  useMyanimelistCallback();

  const username = useMyAnimeListStore((state) => state.username);
  const setUsername = useMyAnimeListStore((state) => state.setUsername);
  const isAuthenticated = useMyAnimeListStore((state) => state.isAuthenticated);
  const isAuthenticating = useMyAnimeListStore(
    (state) => state.isAuthenticating
  );
  const isReauthenticating = useMyAnimeListStore(
    (state) => state.isReauthenticating
  );
  const setIsAuthenticating = useMyAnimeListStore(
    (state) => state.setIsAuthenticating
  );
  const setIsReauthenticating = useMyAnimeListStore(
    (state) => state.setIsReauthenticating
  );

  const {
    register,
    handleSubmit,
    formState: { errors }
  } = useForm<FormData>();

  const onSubmit = async () => {
    setIsAuthenticating(true);
    setIsReauthenticating(true);
    await MyAnimeListService.authorize();
  };

  const isDisabled =
    (isAuthenticating && !isAuthenticated) || isReauthenticating;

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="flex flex-col gap-2">
      <div className="flex gap-1">
        <TextField
          className="max-w-xs w-full"
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

        <Button
          variant="primary"
          size="small"
          type="submit"
          isDisabled={isDisabled}
          isLoading={isDisabled}>
          {!isAuthenticated ? 'Authorize' : 'Re-authorize'}
        </Button>
      </div>

      <div>
        <Button
          variant="ghost"
          onClick={() => openUrl('https://myanimelist.net/register.php')}>
          Create a new MyAnimeList account{' '}
        </Button>
      </div>
    </form>
  );
};

export default MyanimelistForm;
