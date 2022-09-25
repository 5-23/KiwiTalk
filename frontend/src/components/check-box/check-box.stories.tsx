import { Story } from '@storybook/react';
import { ComponentProps } from 'react';

import { CheckBox } from '.';

export default {
  title: 'KiwiTalk/components/CheckBox',
  component: CheckBox,
};

type AdditionalProp = {
  label: string
}

const Template: Story<ComponentProps<typeof CheckBox> & AdditionalProp> = (args) =>
  <CheckBox
    id="example"
    disabled={args.disabled}
    checked={args.checked}
    indeterminate={args.indeterminate}
  >{args.label}</CheckBox>;

export const Default = Template.bind({});
Default.args = {
  label: 'Default Checkbox',
  checked: false,
  indeterminate: false,
  disabled: false,
};

export const Checked = Template.bind({});
Checked.args = {
  label: 'Checked Checkbox',
  checked: true,
  indeterminate: false,
  disabled: false,
};

export const Indeterminate = Template.bind({});
Indeterminate.args = {
  label: 'Indeterminate Checkbox',
  checked: false,
  indeterminate: true,
  disabled: false,
};

export const Disabled = Template.bind({});
Disabled.args = {
  label: 'Disabled Checkbox',
  checked: false,
  indeterminate: false,
  disabled: true,
};
