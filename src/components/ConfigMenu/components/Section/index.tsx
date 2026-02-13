import { Typography } from '@mui/material';
import { FC, ReactNode } from 'react';

interface SectionProps {
  title: string;
  children: ReactNode;
}

const Section: FC<SectionProps> = ({ title, children }) => {
  return (
    <fieldset className="border border-primary/25 rounded-md px-2 pt-2 pb-4 w-full">
      <legend>
        <Typography variant="overline">{title}</Typography>
      </legend>

      {children}
    </fieldset>
  );
};

export default Section;
