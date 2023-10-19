import { style } from '@vanilla-extract/css';

export const loginInput = style({
  display: 'block',
  marginBottom: 12,
});

export const loginButton = style({
  display: 'block',
  marginTop: 12,
  marginLeft: 0,
  marginBottom: 7,
  marginRight: 0,
  width: '100%',
});

export const loginCheckbox = style({
  display: 'block',
  marginTop: 3,
});

export const errorMessage = style({
  color: 'red',
});

export const resetText = style({
  color: '#4D5061',
  textAlign: 'center',
  userSelect: 'none',
});
