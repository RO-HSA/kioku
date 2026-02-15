import { TextField } from '@mui/material';
import { openUrl } from '@tauri-apps/plugin-opener';
import { FC } from 'react';
import { useForm } from 'react-hook-form';

import Button from '@/components/ui/Button';
import useMyanimelistCallback from '@/hooks/integrations/useMyanimelistCallback';
import { Provider } from '@/types/List';
import { buildRegisterUrl } from '@/utils/url';

type FormData = {
  username: string;
};

interface OauthFormProps {
  provider: Provider;
  username: string | null;
  setUsername: (username: string | null) => void;
  isAuthenticated: boolean;
  isAuthenticating: boolean;
  isReauthenticating: boolean;
  setIsAuthenticating: (isAuthenticating: boolean) => void;
  setIsReauthenticating: (isReauthenticating: boolean) => void;
  authorizeFn: () => Promise<void>;
}

const OauthForm: FC<OauthFormProps> = ({
  provider,
  username,
  setUsername,
  isAuthenticated,
  isAuthenticating,
  isReauthenticating,
  setIsAuthenticating,
  setIsReauthenticating,
  authorizeFn
}) => {
  useMyanimelistCallback();

  const {
    register,
    handleSubmit,
    formState: { errors }
  } = useForm<FormData>();

  const onSubmit = async () => {
    setIsAuthenticating(true);
    setIsReauthenticating(true);
    await authorizeFn();
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
          onClick={() => openUrl(buildRegisterUrl(provider))}>
          Create a new {provider} account{' '}
        </Button>
      </div>
    </form>
  );
};

export default OauthForm;
