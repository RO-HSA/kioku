import { openUrl } from '@tauri-apps/plugin-opener';
import { FC } from 'react';

import placeholder from '@/assets/cover-placeholder.svg';
import { cn } from '@/lib/utils';

interface AnimeCoverProps {
  title: string;
  imageUrl?: string;
  url?: string;
}

const AnimeCover: FC<AnimeCoverProps> = ({ imageUrl, title, url }) => {
  const handleOpenInBrowser = async () => {
    if (!url) return;

    await openUrl(url);
  };

  return (
    <div className="p-0.5 border border-dashed border-primary rounded-md inline-block">
      <img
        className={cn(
          'w-36 h-52 object-fill rounded-md',
          url && 'cursor-pointer'
        )}
        src={imageUrl || placeholder}
        alt={title}
        onClick={handleOpenInBrowser}
      />
    </div>
  );
};

export default AnimeCover;
