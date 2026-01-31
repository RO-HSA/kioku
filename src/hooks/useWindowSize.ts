import { useMediaQuery } from '@mui/material';

const useWindowSize = () => {
  const isMobile = useMediaQuery('(max-width: 599px)');

  return { isMobile };
};

export default useWindowSize;
