unsigned int audit_popcount(unsigned int value) {
  unsigned char count = 0;
  for (unsigned char i = 0; i < 16; i++)
    if (value & (1u << i))
      count++;
  return count;
}
