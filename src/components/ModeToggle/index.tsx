import { ChangeEvent } from 'react';
import { useColorScheme } from '@mui/material';

import ModeSwitch from './components/ModeSwitch';
import { useSidebarStore } from '@/stores/sidebar';
import { MoonStar, Sun } from 'lucide-react';
import Button from '../ui/Button';

const ModeToggle = () => {
  const isSidebarOpen = useSidebarStore((state) => state.isOpen);
  const { mode, setMode } = useColorScheme();

  const handleChange = (event: ChangeEvent<HTMLInputElement>) => {
    setMode(event.target.checked ? 'dark' : 'light');
  };

  if (!isSidebarOpen) {
    return (
      <Button
        onClick={() => setMode(mode === 'dark' ? 'light' : 'dark')}
        variant="ghost">
        {mode === 'dark' ? (
          <MoonStar className="size-5" />
        ) : (
          <Sun className="size-5" />
        )}
      </Button>
    );
  }

  return <ModeSwitch checked={mode === 'dark'} onChange={handleChange} />;
};

export default ModeToggle;
