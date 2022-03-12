const main = () => {
  console.log(Date.now());
  for (var i = 0; i < 100000000; i++) {}
  console.log(Date.now());
};

main();
