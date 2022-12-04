import React from 'react';

function ZombiePreview() {
  return (
    <div>
      <div
        className="relative"
        style={{
          backgroundImage: `url('https://cryptozombies.io/course/static/img/tester-bg@2x.png')`,
          width: '47vh',
          height: '84vh',
          backgroundSize: 'cover',
        }}
      ></div>
    </div>
  );
}

export default ZombiePreview;
