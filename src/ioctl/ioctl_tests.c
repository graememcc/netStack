#include <sys/ioctl.h>
#include <sys/socket.h>
#include <linux/if.h>
#include <linux/if_tun.h>
#include <stddef.h>


unsigned long ifr_size() {
  return (unsigned long) sizeof(struct ifreq);
}


unsigned long ifnamesiz() {
  return (unsigned long) IFNAMSIZ;
}


unsigned long ifname_offset() {
  return (unsigned long) offsetof(struct ifreq, ifr_name);
}


unsigned long ifname_member_size() {
  struct ifreq ifr;
  return (unsigned long) sizeof(ifr.ifr_name);
}


unsigned long ifr_flags_offset() {
  return (unsigned long) offsetof(struct ifreq, ifr_flags);
}


unsigned long ifr_flags_size() {
  struct ifreq ifr;
  return (unsigned long) sizeof(ifr.ifr_flags);
}


unsigned long iff_tap() {
  return (unsigned long) IFF_TAP;
}


unsigned long iff_no_pi() {
  return (unsigned long) IFF_NO_PI;
}


unsigned long tunsetiff() {
  return (unsigned long) TUNSETIFF;
}
