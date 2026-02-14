import { cn } from '@/lib/utils';
import { Typography } from '@mui/material';
import { openUrl } from '@tauri-apps/plugin-opener';
import { FC, ReactNode } from 'react';

interface AnimeTitleProps {
  url?: string;
  children: ReactNode;
}

const AnimeTitle: FC<AnimeTitleProps> = ({ url, children }) => {
  const handleOpenInBrowser = async () => {
    if (!url) return;

    await openUrl(url);
  };

  return (
    <div className="flex">
      <span
        className={cn(url ? 'cursor-pointer' : '')}
        onClick={handleOpenInBrowser}>
        <Typography variant="body1" color="primary">
          {children}
        </Typography>
      </span>
    </div>
  );
};

export default AnimeTitle;
