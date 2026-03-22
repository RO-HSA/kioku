import { Divider, Typography } from '@mui/material';
import { FC } from 'react';

interface InfoHeaderProps {
  label: string;
}

const InfoHeader: FC<InfoHeaderProps> = ({ label }) => {
  return (
    <>
      <Typography className="font-bold!" variant="body1">
        {label}
      </Typography>

      <Divider />
    </>
  );
};

export default InfoHeader;
